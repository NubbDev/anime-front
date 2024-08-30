import { Channel, invoke } from '@tauri-apps/api/core';
import {
    ClientWSMessageType,
    ServerWSMessageType,
} from '../misc/enums'


export default class Websocket {
    static readonly listeners: Map<ServerWSMessageType | 'connected' | 'disconnected', Array<(data: any) => void>> = new Map()
    private onConnectListeners = new Array<() => void>()
    private onDisconnectListeners = new Array<() => void>()
    constructor(
        onConnectListeners: Array<() => void>,
        onDisconnectListeners: Array<() => void>
    ) {
        this.onConnectListeners = onConnectListeners
        this.onDisconnectListeners = onDisconnectListeners
    }
    
    static async connect() {
        const onConnectListeners: Array<() => void> = []
        const onDisconnectListeners: Array<() => void> = []

        const onMessage = new Channel<{type: ServerWSMessageType | 'connected' | 'disconnected', data: any | null}>();
        onMessage.onmessage = (msgData) => {
            if (msgData.type === 'connected') {
                for (const l of onConnectListeners) {
                    l()
                }
            } else if (msgData.type === 'disconnected') {
                for (const l of onDisconnectListeners) {
                    l()
                }
            } else {
                if (!Websocket.listeners.has(msgData.type)) return;
                const listedListerners = Websocket.listeners.get(msgData.type)
                for (const l of listedListerners!) {
                    l(msgData.data)
                }
            }
        }
        return await invoke('handle_connection', {onMessage: onMessage}).then(() => 
            new Websocket(onConnectListeners, onDisconnectListeners)
        )
    }

    static async send<T>(type: ClientWSMessageType, data?: T) {
        await invoke('send_message', {messageType: type, data})
    }

    static async disconnect() {
        await invoke('disconnect')
    }

    static addListener<T>(type: ServerWSMessageType, listener: (data: T) => void) {
        if (!this.listeners.has(type)) {
            this.listeners.set(type, [])
        }
        const listedListerners = this.listeners.get(type)
        listedListerners?.push(listener)
    }

    onDisconnect(callback: () => void) {
        this.onDisconnectListeners.push(callback)
    }
    onConnect(callback: () => void) {
        this.onConnectListeners.push(callback)
    }

    


    
  }