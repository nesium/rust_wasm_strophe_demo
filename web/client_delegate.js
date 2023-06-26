export class JSClientDelegate {
    messages_appended(conversation, ids) {
        if (this.onMessagesAppended) {
            this.onMessagesAppended(conversation, ids)
        }
    }
}