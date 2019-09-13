package engine;

import image.ImageCodec;

import java.io.IOException;
import java.nio.file.Path;

/**
 * @author renato.dias
 */
public interface Engine {

    Long scale(int width, int height, Double quality, ImageCodec.IMG_FORMAT format, Path in, Path out) throws Exception;

    String getDescription();
}
