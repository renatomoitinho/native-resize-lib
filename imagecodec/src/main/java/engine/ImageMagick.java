package engine;

import image.ImageCodec;
import org.apache.commons.lang3.StringUtils;
import org.apache.commons.lang3.math.NumberUtils;
import org.im4java.core.ConvertCmd;
import org.im4java.core.IM4JavaException;
import org.im4java.core.IMOperation;
import org.im4java.core.IdentifyCmd;
import org.im4java.process.ArrayListOutputConsumer;
import org.im4java.process.Pipe;

import java.io.ByteArrayInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Instant;
import java.time.temporal.ChronoUnit;

/**
 * @author renato.dias
 */
public class ImageMagick implements Engine {

    private static final int IDENTIFY_WIDTH_RS = 1;
    private static final int IDENTIFY_HEIGHT_RS = 2;


    public ImageInfo getInformation(byte[] bytes) throws Exception {

        InputStream input = new ByteArrayInputStream(bytes);
        ArrayListOutputConsumer output = new ArrayListOutputConsumer();
        Pipe pipeIn = new Pipe(input, null);

        IdentifyCmd identifyCmd = new IdentifyCmd();
        identifyCmd.setInputProvider(pipeIn);
        identifyCmd.setOutputConsumer(output);

        IMOperation op = new IMOperation();
        op.ping();
        op.format("%m:%[fx:w]:%[fx:h]");
        op.addImage("-");

        try {
            identifyCmd.run(op);
            String[] out = StringUtils.split(output.getOutput().get(0), ":", 3);

            return new ImageInfo(
                    getPoint(out[IDENTIFY_WIDTH_RS]),
                    getPoint(out[IDENTIFY_HEIGHT_RS]),
                    bytes.length);

        } finally {
            input.close();
        }
    }


    public Boolean scale(InputStream input, OutputStream output, int maxDimension, String extension, Double quality) {
        Pipe pipeIn = new Pipe(input, null);
        Pipe pipeOut = new Pipe(null, output);

        ConvertCmd convert = new ConvertCmd();

        convert.setSearchPath("/usr/bin");
        convert.setInputProvider(pipeIn);
        convert.setOutputConsumer(pipeOut);

        IMOperation op = new IMOperation();

        op.addImage("-");
        op.strip();
        op.background("white");
        op.flatten();
        op.gravity("center");
        op.resize(maxDimension, maxDimension);
        op.layers("coalesce");
        op.interlace("Plane");
        op.quality(quality);
        op.extent(maxDimension, maxDimension);
        op.addImage(extension);

        try {
            convert.run(op);
            return true;
        } catch (IOException | InterruptedException | IM4JavaException e) {
            throw new RuntimeException("ImageMagickEngine, Error ao realizar rezizer ", e);
        } finally {


            try {

                if(input!= null) {
                    input.close();
                }

                if(output!=null) {
                    output.close();
                }

            } catch (IOException ignore) { }
        }
    }


    private Integer getPoint(String number) {
        return NumberUtils.createNumber(number).intValue();
    }


    @Override
    public Long scale(int width, int height, Double quality, ImageCodec.IMG_FORMAT format, Path in, Path out) throws Exception {

        Instant init = Instant.now();

        byte[] bytes = Files.readAllBytes(in);
        OutputStream fout = new FileOutputStream(out.toFile());

        Instant start = Instant.now();

        ImageInfo info = getInformation(bytes);

        println("elapsed load buffer => size[%sx%s] %s thread=[%s] [%sms]", info.getWidth(), info.getHeight(), Thread.currentThread().getName(), in.toFile().getName(),
                ChronoUnit.MILLIS.between(start, Instant.now()));

        start = Instant.now();

        InputStream inputStream = new ByteArrayInputStream(bytes);

        scale(inputStream, fout, Math.max(width, height), String.format("%s:-", format.name().toLowerCase()),  quality);

        println("elapsed scale buffer => size[%sx%s] %s thread=[%s] [%sms]",  width, height, Thread.currentThread().getName(),
                in.toFile().getName(), ChronoUnit.MILLIS.between(start, Instant.now()));

        return ChronoUnit.MILLIS.between(init, Instant.now());
    }

    private void println(String message, int width, int height, String thread, String name, long time) {
        System.out.println(String.format(message, width, height, thread, name, time));
    }

    @Override
    public String getDescription() {
        return "::. Running ImageMagick engine .::";
    }
}
