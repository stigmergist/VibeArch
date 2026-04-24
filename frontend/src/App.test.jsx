import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, afterEach, describe, expect, it, vi } from 'vitest';
import App from './App';

class MockWebSocket {
  static instances = [];
  static OPEN = 1;
  static CLOSED = 3;

  constructor(url) {
    this.url = url;
    this.readyState = 0;
    this.sent = [];
    this.onopen = null;
    this.onclose = null;
    this.onerror = null;
    this.onmessage = null;
    MockWebSocket.instances.push(this);
  }

  send(payload) {
    this.sent.push(payload);
  }

  close() {
    this.readyState = MockWebSocket.CLOSED;
  }

  open() {
    this.readyState = MockWebSocket.OPEN;
    this.onopen?.();
  }

  fail() {
    this.onerror?.(new Event('error'));
  }

  drop() {
    this.readyState = MockWebSocket.CLOSED;
    this.onclose?.(new CloseEvent('close'));
  }

  receive(payload) {
    this.onmessage?.({ data: JSON.stringify(payload) });
  }
}

describe('App', () => {
  beforeEach(() => {
    MockWebSocket.instances = [];
    global.fetch = vi.fn(async (url, options = {}) => {
      if (String(url).endsWith('/auth/login')) {
        return {
          ok: true,
          json: async () => ({
            token: 'session-token',
            username: 'alice',
            displayName: 'Alice',
            expiresAt: '2030-01-01T00:00:00Z',
          }),
        };
      }
      if (String(url).includes('/auth/messages')) {
        return {
          ok: true,
          json: async () => ({
            messages: [
              {
                id: 'message-1',
                type: 'message',
                sender: 'Bob',
                text: 'Earlier hello',
                sentAt: '2030-01-01T00:00:00Z',
              },
            ],
            hasMore: false,
            nextBefore: null,
          }),
        };
      }
      if (String(url).endsWith('/auth/logout')) {
        return {
          ok: true,
          json: async () => ({}),
        };
      }
      throw new Error(`Unexpected fetch call: ${url}`);
    });
    global.WebSocket = MockWebSocket;
  });

  afterEach(() => {
    cleanup();
    vi.restoreAllMocks();
  });

  async function signIn() {
    render(<App />);

    fireEvent.change(screen.getByLabelText('Username'), { target: { value: 'alice' } });
    fireEvent.change(screen.getByLabelText('Password'), { target: { value: 'password123' } });
    fireEvent.click(screen.getByRole('button', { name: 'Sign in' }));

    await waitFor(() => expect(MockWebSocket.instances).toHaveLength(1));
    await waitFor(() => expect(MockWebSocket.instances[0].onopen).toBeTypeOf('function'));
    return MockWebSocket.instances[0];
  }

  it('reconnects after an unexpected socket close and shows received messages', async () => {
    const firstSocket = await signIn();
    await act(async () => {
      firstSocket.open();
    });
    await waitFor(() => expect(screen.getByText('Signed in as Alice.')).toBeInTheDocument());
    expect(screen.getByText('Earlier hello')).toBeInTheDocument();

    await act(async () => {
      firstSocket.drop();
    });
    await waitFor(() =>
      expect(screen.getByText('Connection lost. Reconnecting (1/3)...')).toBeInTheDocument()
    );

    await waitFor(() => expect(MockWebSocket.instances).toHaveLength(2), { timeout: 2000 });

    const secondSocket = MockWebSocket.instances[1];
    await act(async () => {
      secondSocket.open();
      secondSocket.receive({
        type: 'message',
        sender: 'Alice',
        text: 'Welcome back',
        sentAt: '2030-01-01T00:00:00Z',
      });
    });

    await waitFor(() => expect(screen.getByText('Welcome back')).toBeInTheDocument());
  });

  it('sends chat payloads with only the text field', async () => {
    const socket = await signIn();
    await act(async () => {
      socket.open();
    });

    fireEvent.change(screen.getByPlaceholderText('Type your message...'), {
      target: { value: 'hello there' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Send' }));

    expect(socket.sent).toEqual([JSON.stringify({ text: 'hello there' })]);
  });

  it('loads older messages only when scrolling near the top', async () => {
    global.fetch = vi.fn(async (url, options = {}) => {
      if (String(url).endsWith('/auth/login')) {
        return {
          ok: true,
          json: async () => ({
            token: 'session-token',
            username: 'alice',
            displayName: 'Alice',
            expiresAt: '2030-01-01T00:00:00Z',
          }),
        };
      }
      if (String(url).includes('/auth/messages?limit=25&before=message-2')) {
        return {
          ok: true,
          json: async () => ({
            messages: [
              {
                id: 'message-1',
                type: 'message',
                sender: 'Bob',
                text: 'Oldest message',
                sentAt: '2030-01-01T00:00:00Z',
              },
            ],
            hasMore: false,
            nextBefore: null,
          }),
        };
      }
      if (String(url).includes('/auth/messages?limit=25')) {
        return {
          ok: true,
          json: async () => ({
            messages: [
              {
                id: 'message-2',
                type: 'message',
                sender: 'Carol',
                text: 'Newest saved message',
                sentAt: '2030-01-01T00:01:00Z',
              },
            ],
            hasMore: true,
            nextBefore: 'message-2',
          }),
        };
      }
      if (String(url).endsWith('/auth/logout')) {
        return {
          ok: true,
          json: async () => ({}),
        };
      }
      throw new Error(`Unexpected fetch call: ${url}`);
    });

    const socket = await signIn();
    await act(async () => {
      socket.open();
    });

    const list = screen.getByRole('list');
    Object.defineProperty(list, 'scrollHeight', { value: 500, configurable: true });
    Object.defineProperty(list, 'clientHeight', { value: 200, configurable: true });

    await waitFor(() => expect(screen.getByText('Newest saved message')).toBeInTheDocument());

    list.scrollTop = 150;
    fireEvent.scroll(list);
    expect(global.fetch).toHaveBeenCalledTimes(2);

    list.scrollTop = 10;
    fireEvent.scroll(list);

    await waitFor(() => expect(screen.getByText('Oldest message')).toBeInTheDocument());
  });
});