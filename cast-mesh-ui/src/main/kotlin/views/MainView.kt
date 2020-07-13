package views

import tornadofx.*

class MainView : View() {
    override val root = vbox {
        button("Send")
        label("Send message to device")
    }
}