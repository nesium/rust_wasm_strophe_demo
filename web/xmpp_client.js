export class JSXMPPClient {
    constructor() {
        this.__connection = new Strophe.Connection(
            "wss://chat.prose.org/websocket/",
            { protocol: "wss" }
        );
        this.__connection.rawInput = (data) => {
            //console.log("RECV", data);
            if (this.__handlers) {
                this.__handlers.handle_stanza(data);
            }
        };
        this.__connection.rawOutput = (data) => {
            //console.log("SENT", data);
        }
    }

    set_handlers(handlers) {
        console.log("Setting handlers…");
        this.__handlers = handlers;
    }
    connect(jid, password) {
        console.log("Connecting via StropheJS…");

        return new Promise ((resolve, reject) => {
            this.__connection.connect(jid, password, (status) => {
                console.log(arguments);

                if (status === Strophe.Status.CONNECTING) {
                    console.log("Strophe is connecting.");
                } else if (status === Strophe.Status.CONNFAIL) {
                    console.log("Strophe failed to connect.");
                    reject(new Error("Something went wrong."))
                } else if (status === Strophe.Status.DISCONNECTING) {
                    console.log("Strophe is disconnecting.");
                } else if (status === Strophe.Status.DISCONNECTED) {
                    console.log("Strophe is disconnected.");
                } else if (status === Strophe.Status.CONNECTED) {
                    console.log("Strophe is connected.");
                    resolve()

                    //connection.addHandler(onMessage, null, 'message', null, null,  null);
                    //connection.send($pres().tree());
                }
            });
        })
    }

    disconnect() {
        this.__connection.disconnect();
    }

    send_stanza(stanza) {
        console.log("Sending stanza", stanza);
        let element = new DOMParser().parseFromString(stanza, "text/xml").firstElementChild;
        this.__connection.send(element);
    }
}
