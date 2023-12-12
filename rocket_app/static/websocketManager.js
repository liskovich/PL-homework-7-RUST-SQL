class WebSocketManager {
  constructor(url) {
    if (!WebSocketManager.instance) {
      this.url = url;
      this.socket = new WebSocket(this.url);
      WebSocketManager.instance = this;
    }

    return WebSocketManager.instance;
  }

  static getInstance(url) {
    return WebSocketManager.instance || new WebSocketManager(url);
  }

  // handle WebSocket events
  handleWebSocketEvents(updateCallback) {
    this.socket.addEventListener('message', function (event) {
      updateCallback(event.data);
    });
  }

  closeConnection() {
    this.socket.close();
  }
}