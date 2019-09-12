
import java.io.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.Instant;
import java.time.temporal.ChronoUnit;
import java.util.Optional;
import java.util.UUID;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;
import java.util.stream.Stream;

/**
 * @author renato.dias
 */
public class RunTest {

    private static final ExecutorService C_EXECUTOR_SERVICE = Executors.newFixedThreadPool(20);

    public static void main(String[] args) throws IOException {

        Path pathJpeg = getOptionalPath(args).orElse(Paths.get("/tmp"));

        if (!(Files.exists(pathJpeg) && Files.isDirectory(pathJpeg))) {
            throw new IllegalArgumentException("Directories images not found");
        }

        File[] images = Stream.of(pathJpeg.toFile().listFiles(File::isFile)).toArray(File[]::new);

        for (final File img : images) {

            if (!img.getName().matches("^.*(jpe?g|png)$"))
                continue;

            String newName = UUID.randomUUID().toString() + "__" + img.getName();
            final Path out = img.toPath().getParent().resolve(newName);

            C_EXECUTOR_SERVICE.submit(() -> {
                try {
                    resize_perform(img.toPath(), out);
                } catch (IOException e) {
                    System.err.println(e.getMessage());
                }
            });
        }

        C_EXECUTOR_SERVICE.shutdown();
        try {
            if (!C_EXECUTOR_SERVICE.awaitTermination(60, TimeUnit.SECONDS)) {
                C_EXECUTOR_SERVICE.shutdownNow();
            }
        } catch (InterruptedException ex) {
            C_EXECUTOR_SERVICE.shutdownNow();
            Thread.currentThread().interrupt();
        }
    }

    private static void resize_perform(Path in, Path out) throws IOException {

        byte[] bytes = Files.readAllBytes(in);
        OutputStream fout = new FileOutputStream(out.toFile());

        Instant start = Instant.now();

        HelloWorld hello = new HelloWorld(bytes);

        println("elapsed load buffer => %s thread=[%s] [%sms]", Thread.currentThread().getName(), in.toFile().getName(),
                ChronoUnit.MILLIS.between(start, Instant.now()));

        start = Instant.now();

        bytes = hello.scale(1024, 1024, 80, ImageCodec.IMG_FORMAT.JPEG);

        println("elapsed scale buffer => %s thread=[%s] [%sms]", Thread.currentThread().getName(),
                in.toFile().getName(), ChronoUnit.MILLIS.between(start, Instant.now()));

        fout.write(bytes);
        fout.close();
        hello.close();
    }

    private static Optional<Path> getOptionalPath(String[] args) {

        if (args.length == 0) {
            return Optional.empty();
        }

        return Optional.of(Paths.get(args[0]));
    }

    private static void println(String message, String thread, String name, long time) {
        System.out.println(String.format(message, thread, name, time));
    }

}
