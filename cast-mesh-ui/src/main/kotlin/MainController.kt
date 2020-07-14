
import com.tylerlowrey.grpc.registration.RegisterDeviceGrpcKt
import com.tylerlowrey.grpc.registration.Registration
import io.grpc.ManagedChannelBuilder
import kotlinx.coroutines.runBlocking
import tornadofx.*

class MainController : Controller() {



    fun registerDevice(deviceAddress: String) {
        var response: Registration.RegistrationResult

        runAsync {
            val channel = ManagedChannelBuilder.forAddress("192.168.1.131", 50051).build()
            val stub = RegisterDeviceGrpcKt.RegisterDeviceCoroutineStub(channel)
            val registrationRequest = Registration.RegistrationMessage.newBuilder()
                    .setDeviceAddress("192.168.1.131:3005")
                    .build()
            runBlocking {
                response = stub.send(registrationRequest)
                println(response)
            }
        } ui {
            println("Response received")
        }

    }
}