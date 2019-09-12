import java.io.IOException;
import java.io.OutputStream;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.time.Instant;
import java.time.temporal.ChronoUnit;

/**
 * @author renato.dias
 */
public class RunTest {

    public static void main(String[] args) throws IOException {

        String[] images = {

        };



        byte[] bytes = Files.readAllBytes(Paths.get("/home/renato/Documents/repositories/rust/imgs/media440.jpeg"));

        OutputStream out = Files.newOutputStream(Paths.get("/home/renato/Documents/repositories/rust/imgs/out.jpeg"));

        Instant start = Instant.now();

        HelloWorld hello = new HelloWorld(bytes);

        System.out.println("elapsed load buffer => " + ChronoUnit.MILLIS.between(start, Instant.now()) + " ms");

        start = Instant.now();

        bytes = hello.scale(1024, 1024, 80, ImageCodec.IMG_FORMAT.JPEG);

        System.out.println("elapsed scale buffer => " + ChronoUnit.MILLIS.between(start, Instant.now()) + " ms");

        out.write(bytes);

        out.close();
        hello.close();
    }
}
