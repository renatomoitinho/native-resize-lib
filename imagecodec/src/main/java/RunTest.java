
import engine.CustomEngine;
import engine.Engine;
import engine.ImageMagick;
import image.ImageCodec;

import java.io.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Optional;
import java.util.UUID;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicLong;
import java.util.stream.Stream;

/**
 * @author renato.dias
 */
public class RunTest {

    private static final ExecutorService C_EXECUTOR_SERVICE = Executors.newFixedThreadPool(20);

    public static void main(String[] args) throws Exception {

        Path pathJpeg = getOptionalPath(args).orElse(Paths.get("/tmp"));

        Engine engine = getEngine(args).orElseThrow(()-> new IllegalArgumentException("Not engine found "));

        if (!(Files.exists(pathJpeg) && Files.isDirectory(pathJpeg))) {
            throw new IllegalArgumentException("Directories images not found");
        }

        System.out.println(">>>>>>" + engine.getDescription() + "<<<<<<<<");

        File[] images = Stream.of(pathJpeg.toFile().listFiles(File::isFile)).toArray(File[]::new);

        final AtomicLong aLong = new AtomicLong(0);
        
        System.out.println("Quantidade de imagens : " + images.length);

        for (final File img : images) {

            if (!img.getName().matches("^.*(jpe?g|png)$"))
                continue;

            String newName = UUID.randomUUID().toString() + "__" + img.getName();
            final Path out = Paths.get("/tmp").resolve(newName);

            C_EXECUTOR_SERVICE.submit(() -> {
                try {
                    final Long elapsed = engine.scale (1024, 1024 , 80.0, ImageCodec.IMG_FORMAT.JPG, img.toPath(), out);
                    aLong.addAndGet(elapsed);
                } catch (Exception e) {
                    System.err.println(e.getMessage());
                }
            });
        }

        C_EXECUTOR_SERVICE.shutdown();
        try {
            if (!C_EXECUTOR_SERVICE.awaitTermination(60, TimeUnit.SECONDS)) {
                C_EXECUTOR_SERVICE.shutdownNow();
            }

            System.out.println("total time count " + aLong.get() + " ms");
        } catch (InterruptedException ex) {
            C_EXECUTOR_SERVICE.shutdownNow();
            Thread.currentThread().interrupt();
        }

    }

    private static Optional<Path> getOptionalPath(String[] args) {

        if (args.length == 0) {
            return Optional.empty();
        }

        return Optional.of(Paths.get(args[0]));
    }

    private static Optional<Engine> getEngine (String[] args) {

        if(args.length < 2) {
            return Optional.of(new CustomEngine());
        }

        boolean active= args[1].matches("true");

        return active ? Optional.of(new ImageMagick()) : Optional.of(new CustomEngine());
    }


    private static void println(String message, String thread, String name, long time) {
        System.out.println(String.format(message, thread, name, time));
    }

}
