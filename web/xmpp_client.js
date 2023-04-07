export class JSXMPPClient {
    constructor() {
        this.__connection = new Strophe.Connection(
            "wss://chat.prose.org/websocket/",
            { protocol: "wss" }
        );
    }
    connect(jid, password) {
        console.log("CONNECT!!")
        this.__connection.connect(jid, password, JSXMPPClient.onConnect);
    }

    static onConnect(status) {
        if (status == Strophe.Status.CONNECTING) {
            console.log("Strophe is connecting.");
        } else if (status == Strophe.Status.CONNFAIL) {
            console.log("Strophe failed to connect.");
        } else if (status == Strophe.Status.DISCONNECTING) {
            console.log("Strophe is disconnecting.");
        } else if (status == Strophe.Status.DISCONNECTED) {
            console.log("Strophe is disconnected.");
        } else if (status == Strophe.Status.CONNECTED) {
            console.log("Strophe is connected.");

            //connection.addHandler(onMessage, null, 'message', null, null,  null);
            //connection.send($pres().tree());
        }
    }
}
