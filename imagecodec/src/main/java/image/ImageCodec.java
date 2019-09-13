package image;

public interface ImageCodec {
    enum IMG_FORMAT {
        JPEG, JPG, PNG
    }

    byte[] resize(int width, int height, int quality, IMG_FORMAT img_format);

    byte[] scale(int width, int height, int quality, IMG_FORMAT img_format);
}
