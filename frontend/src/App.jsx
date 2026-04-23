import { useEffect, useMemo, useRef, useState } from 'react';

const socketUrl = 'ws://localhost:8000/ws/chat';

function formatTime(iso) {
  const date = new Date(iso);
  return new Intl.DateTimeFormat('en-US', {
    hour: 'numeric',
    minute: '2-digit',
  }).format(date);
}

export default function App() {
  const [name, setName] = useState('Guest');
  const [draft, setDraft] = useState('');
  const [messages, setMessages] = useState([]);
  const [connected, setConnected] = useState(false);
  const socketRef = useRef(null);
  const listRef = useRef(null);

  useEffect(() => {
    const socket = new WebSocket(socketUrl);
    socketRef.current = socket;

    socket.onopen = () => setConnected(true);
    socket.onclose = () => setConnected(false);
    socket.onerror = () => setConnected(false);

    socket.onmessage = (event) => {
      const payload = JSON.parse(event.data);
      setMessages((current) => [...current, payload]);
    };

    return () => {
      socket.close();
    };
  }, []);

  useEffect(() => {
    if (!listRef.current) {
      return;
    }
    listRef.current.scrollTop = listRef.current.scrollHeight;
  }, [messages]);

  const statusLabel = useMemo(() => {
    return connected ? 'Connected' : 'Disconnected';
  }, [connected]);

  const sendMessage = (event) => {
    event.preventDefault();

    const text = draft.trim();
    if (!text || !socketRef.current || socketRef.current.readyState !== WebSocket.OPEN) {
      return;
    }

    socketRef.current.send(
      JSON.stringify({
        sender: name.trim() || 'Guest',
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
          <label htmlFor="name">Display name</label>
          <input
            id="name"
            value={name}
            onChange={(event) => setName(event.target.value)}
            maxLength={24}
          />
        </div>

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
