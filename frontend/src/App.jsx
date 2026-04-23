import { useEffect, useMemo, useRef, useState } from 'react';

const socketUrl = import.meta.env.VITE_CHAT_WS_URL || 'ws://localhost:8000/ws/chat';
const authBaseUrl =
  import.meta.env.VITE_AUTH_BASE_URL || socketUrl.replace(/^ws/i, 'http').replace(/\/ws\/chat$/, '/auth');

function formatTime(iso) {
  const date = new Date(iso);
  return new Intl.DateTimeFormat('en-US', {
    hour: 'numeric',
    minute: '2-digit',
  }).format(date);
}

export default function App() {
  const [mode, setMode] = useState('login');
  const [username, setUsername] = useState('');
  const [displayName, setDisplayName] = useState('');
  const [password, setPassword] = useState('');
  const [draft, setDraft] = useState('');
  const [messages, setMessages] = useState([]);
  const [connected, setConnected] = useState(false);
  const [session, setSession] = useState(null);
  const [statusMessage, setStatusMessage] = useState('Sign in to join the chat.');
  const [isAuthenticating, setIsAuthenticating] = useState(false);
  const socketRef = useRef(null);
  const listRef = useRef(null);

  useEffect(() => {
    if (!session) {
      return undefined;
    }

    let shouldResetSession = true;
    const socket = new WebSocket(`${socketUrl}?token=${encodeURIComponent(session.token)}`);
    socketRef.current = socket;

    socket.onopen = () => {
      setConnected(true);
      setStatusMessage(`Signed in as ${session.displayName}.`);
    };
    socket.onclose = () => {
      setConnected(false);
      socketRef.current = null;
      if (shouldResetSession) {
        setSession(null);
        setStatusMessage('Connection closed. Sign in again to continue.');
      }
    };
    socket.onerror = () => {
      setConnected(false);
      setStatusMessage('Unable to connect to chat. Check backend auth/session availability.');
    };

    socket.onmessage = (event) => {
      const payload = JSON.parse(event.data);
      setMessages((current) => [...current, payload]);
    };

    return () => {
      shouldResetSession = false;
      socket.close();
    };
  }, [session]);

  useEffect(() => {
    if (!listRef.current) {
      return;
    }
    listRef.current.scrollTop = listRef.current.scrollHeight;
  }, [messages]);

  const statusLabel = useMemo(() => {
    return connected ? 'Connected' : 'Disconnected';
  }, [connected]);

  const authenticate = async (event) => {
    event.preventDefault();

    if (!username.trim() || !password.trim() || (mode === 'register' && !displayName.trim())) {
      setStatusMessage('Provide the required credentials before continuing.');
      return;
    }

    setIsAuthenticating(true);
    setStatusMessage(mode === 'register' ? 'Creating account...' : 'Signing in...');

    try {
      const response = await fetch(`${authBaseUrl}/${mode}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(
          mode === 'register'
            ? {
                username,
                password,
                displayName,
              }
            : {
                username,
                password,
              }
        ),
      });

      const payload = await response.json();
      if (!response.ok) {
        throw new Error(payload.detail || 'Authentication failed.');
      }

      setSession(payload);
      setDisplayName(payload.displayName);
      setPassword('');
      setMessages([]);
    } catch (error) {
      setSession(null);
      setStatusMessage(error instanceof Error ? error.message : 'Authentication failed.');
    } finally {
      setIsAuthenticating(false);
    }
  };

  const logout = async () => {
    if (!session) {
      return;
    }

    setStatusMessage('Signing out...');

    try {
      await fetch(`${authBaseUrl}/logout`, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${session.token}`,
        },
      });
    } finally {
      setConnected(false);
      setSession(null);
      setMessages([]);
      setDraft('');
      setPassword('');
      setStatusMessage('Signed out.');
    }
  };

  const sendMessage = (event) => {
    event.preventDefault();

    const text = draft.trim();
    if (!text || !socketRef.current || socketRef.current.readyState !== WebSocket.OPEN) {
      return;
    }

    socketRef.current.send(
      JSON.stringify({
        text,
      })
    );

    setDraft('');
  };

  return (
    <main className="shell">
      <section className="chat-card">
        <header className="chat-header">
          <div>
            <p className="eyebrow">Real-time room</p>
            <h1>Simple Chat</h1>
          </div>
          <span className={`status ${connected ? 'online' : 'offline'}`}>{statusLabel}</span>
        </header>

        <div className="name-row">
          <label htmlFor="username">Username</label>
          <input
            id="username"
            value={username}
            onChange={(event) => setUsername(event.target.value)}
            maxLength={24}
            disabled={connected || isAuthenticating}
          />
        </div>

        <div className="name-row">
          <label htmlFor="password">Password</label>
          <input
            id="password"
            type="password"
            value={password}
            onChange={(event) => setPassword(event.target.value)}
            minLength={8}
            disabled={connected || isAuthenticating}
          />
        </div>

        {mode === 'register' && (
          <div className="name-row">
            <label htmlFor="display-name">Display name</label>
            <input
              id="display-name"
              value={displayName}
              onChange={(event) => setDisplayName(event.target.value)}
              maxLength={48}
              disabled={connected || isAuthenticating}
            />
          </div>
        )}

        <form className="composer" onSubmit={authenticate}>
          <button type="submit" disabled={connected || isAuthenticating}>
            {mode === 'register' ? 'Create account' : 'Sign in'}
          </button>
          <button
            type="button"
            disabled={connected || isAuthenticating}
            onClick={() => {
              setMode((current) => (current === 'register' ? 'login' : 'register'));
              setStatusMessage('');
            }}
          >
            {mode === 'register' ? 'Use existing account' : 'Create new account'}
          </button>
          <button type="button" disabled={!session} onClick={logout}>
            Sign out
          </button>
        </form>

        <p className="eyebrow">{statusMessage}</p>

        <ul className="messages" ref={listRef}>
          {messages.length === 0 && <li className="empty">No messages yet. Say hello.</li>}
          {messages.map((message, index) => (
            <li key={`${message.sentAt}-${index}`} className={message.type === 'system' ? 'system' : message.type === 'error' ? 'error-msg' : ''}>
              {message.type === 'system' || message.type === 'error' ? (
                <p>{message.text}</p>
              ) : (
                <>
                  <div className="meta">
                    <strong>{message.sender}</strong>
                    <time>{formatTime(message.sentAt)}</time>
                  </div>
                  <p>{message.text}</p>
                </>
              )}
            </li>
          ))}
        </ul>

        <form className="composer" onSubmit={sendMessage}>
          <input
            placeholder="Type your message..."
            value={draft}
            onChange={(event) => setDraft(event.target.value)}
            disabled={!connected}
          />
          <button type="submit" disabled={!connected || draft.trim().length === 0}>
            Send
          </button>
        </form>
      </section>
    </main>
  );
}
