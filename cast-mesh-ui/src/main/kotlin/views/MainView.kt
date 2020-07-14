package views

import MainController
import tornadofx.*

class MainView : View() {
    val controller: MainController by inject()

    override val root = vbox {
        button("Send") {
            action {
                controller.registerDevice("192.168.1.131:44004")
            }
        }
        label("Send message to device")
    }
}