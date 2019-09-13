package engine;

import image.HelloWorld;
import image.ImageCodec;

import java.io.FileOutputStream;
import java.io.IOException;
import java.io.OutputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Instant;
import java.time.temporal.ChronoUnit;

/**
 * @author renato.dias
 */
public class CustomEngine implements Engine {

    @Override
    public Long scale(int width, int height, Double quality, ImageCodec.IMG_FORMAT format, Path in, Path out) throws IOException {
        
        byte[] bytes = Files.readAllBytes(in);
        OutputStream fout = new FileOutputStream(out.toFile());

        Instant init = Instant.now();
        Instant start = Instant.now();

        HelloWorld hello = new HelloWorld(bytes);

        println("elapsed load buffer => size[%sx%s] %s thread=[%s] [%sms]", hello.getWidth(), hello.getHeight(), Thread.currentThread().getName(), in.toFile().getName(),
                ChronoUnit.MILLIS.between(start, Instant.now()));

        start = Instant.now();

        bytes = hello.scale(width, height, quality.intValue(), format);

        println("elapsed scale buffer => size[%sx%s] %s thread=[%s] [%sms]",  hello.getWidth(), hello.getHeight(), Thread.currentThread().getName(),
                in.toFile().getName(), ChronoUnit.MILLIS.between(start, Instant.now()));

        long end = ChronoUnit.MILLIS.between(init, Instant.now());

        fout.write(bytes);
        fout.close();
        hello.close();

        return end;
    }

    private void println(String message, int width, int height, String thread, String name, long time) {
        System.out.println(String.format(message, width, height, thread, name, time));
    }

    @Override
    public String getDescription() {
        return "::. Running Custom openCv engine .::";
    }
}
