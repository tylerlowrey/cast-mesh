package views

import MainController
import tornadofx.*

class MainView : View() {
    val controller: MainController by inject()

    override val root = vbox {
        button("Send") {
            action {
                controller.sendPing("Ping!")
            }
        }
        label("Send message to device")
    }
}