import unittest
from datetime import datetime, timedelta, timezone

from fastapi.testclient import TestClient
from starlette.websockets import WebSocketDisconnect

from app import main


class FakeClock:
    def __init__(self) -> None:
        self.current = datetime(2026, 4, 23, tzinfo=timezone.utc)

    def now(self) -> datetime:
        return self.current

    def advance(self, *, seconds: int) -> None:
        self.current += timedelta(seconds=seconds)


class AuthLifecycleTests(unittest.TestCase):
    allowed_origin = "http://localhost:5173"

    def setUp(self) -> None:
        self.clock = FakeClock()
        main.manager = main.ConnectionManager()
        main.users = main.UserStore()
        main.sessions = main.SessionStore(session_ttl_seconds=5, now_fn=self.clock.now)
        self.client = TestClient(main.app)

    def tearDown(self) -> None:
        self.client.close()

    def register_user(self) -> dict:
        response = self.client.post(
            "/auth/register",
            json={
                "username": "alice",
                "password": "password123",
                "displayName": "Alice",
            },
            headers={"Origin": self.allowed_origin},
        )
        self.assertEqual(response.status_code, 201)
        return response.json()

    def test_register_returns_expiry_and_cors_headers_for_allowed_origin(self) -> None:
        payload = self.register_user()

        self.assertIn("expiresAt", payload)
        self.assertEqual(
            self.client.post(
                "/auth/login",
                json={"username": "alice", "password": "password123"},
                headers={"Origin": self.allowed_origin},
            ).headers.get("access-control-allow-origin"),
            self.allowed_origin,
        )

    def test_logout_revokes_token(self) -> None:
        payload = self.register_user()
        logout_response = self.client.post(
            "/auth/logout",
            headers={
                "Authorization": f"Bearer {payload['token']}",
                "Origin": self.allowed_origin,
            },
        )

        self.assertEqual(logout_response.status_code, 204)

        with self.assertRaises(WebSocketDisconnect) as exc:
            with self.client.websocket_connect(
                f"/ws/chat?token={payload['token']}",
                headers={"origin": self.allowed_origin},
            ):
                pass

        self.assertEqual(exc.exception.code, 1008)

    def test_session_expiry_rejects_websocket(self) -> None:
        payload = self.register_user()
        self.clock.advance(seconds=6)

        with self.assertRaises(WebSocketDisconnect) as exc:
            with self.client.websocket_connect(
                f"/ws/chat?token={payload['token']}",
                headers={"origin": self.allowed_origin},
            ):
                pass

        self.assertEqual(exc.exception.code, 1008)

    def test_disallowed_websocket_origin_is_rejected(self) -> None:
        payload = self.register_user()

        with self.assertRaises(WebSocketDisconnect) as exc:
            with self.client.websocket_connect(
                f"/ws/chat?token={payload['token']}",
                headers={"origin": "http://evil.example"},
            ):
                pass

        self.assertEqual(exc.exception.code, 1008)


if __name__ == "__main__":
    unittest.main()