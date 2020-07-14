import grpc.PingPongGrpcKt
import tornadofx.*;

class MainController : Controller() {

    fun registerDevice(deviceAddress: String) {
        runAsync {

        } ui {
            println("Ping sent.")
        }

    }
}