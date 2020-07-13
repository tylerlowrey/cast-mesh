import tornadofx.*;
import views.MainView

class Application : App(MainView::class)

fun main(args: Array<String>) {
   launch<Application>(args)
}