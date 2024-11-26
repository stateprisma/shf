type ConnectionCallback = (connected: boolean) => void;
type MessageCallback = (message: string) => void;

class SocketManager {
	private static instance: SocketManager;
	private socket: WebSocket | null = null;
	private connection_callbacks: Set<ConnectionCallback> = new Set();
	private message_callbacks: Set<MessageCallback> = new Set();

	private constructor() {}

	static getInstance(): SocketManager {
		if (!SocketManager.instance) {
			SocketManager.instance = new SocketManager();
		}
		return SocketManager.instance;
	}

	connect(url: string): void {
		if (this.socket) {
			console.warn('WebSocket is already connected.');
			return;
		}

		this.socket = new WebSocket(url);

		this.socket.onopen = () => {
			console.log('WebSocket connected');
			this.notifyConnectionSubscribers(true);
		};

		this.socket.onclose = () => {
			console.log('WebSocket disconnected');
			this.notifyConnectionSubscribers(false);
			this.socket = null;
		};

		this.socket.onerror = (error) => {
			console.error('WebSocket error:', error);
		};

		this.socket.onmessage = (event) => {
			console.log('WebSocket message received:', event.data);
			this.notifyMessageSubscribers(event.data);
		};
	}

	disconnect(): void {
		if (this.socket) {
			this.socket.close();
			this.socket = null;
		}
	}

	send(message: string | ArrayBufferLike | Blob | ArrayBufferView): void {
		if (this.socket && this.socket.readyState === WebSocket.OPEN) {
			this.socket.send(message);
		} else {
			console.error('Cannot send message: WebSocket is not connected.');
		}
	}

	subscribeConnection(callback: ConnectionCallback): void {
		this.connection_callbacks.add(callback);
	}

	unsubscribeConnection(callback: ConnectionCallback): void {
		this.connection_callbacks.delete(callback);
	}

	subscribeMessages(callback: MessageCallback): void {
		this.message_callbacks.add(callback);
	}

	unsubscribeMessages(callback: MessageCallback): void {
		this.message_callbacks.delete(callback);
	}

	private notifyConnectionSubscribers(connected: boolean): void {
		this.connection_callbacks.forEach((callback) => callback(connected));
	}

	private notifyMessageSubscribers(message: string): void {
		this.message_callbacks.forEach((callback) => callback(message));
	}
}

export const socketManager = SocketManager.getInstance();
