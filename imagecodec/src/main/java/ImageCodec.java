public interface ImageCodec {
    enum IMG_FORMAT {
        JPEG, JPG, PNG, WEBP
    }

    byte[] resize(int width, int height, int quality, IMG_FORMAT img_format);

    byte[] scale(int width, int height, int quality, IMG_FORMAT img_format, int[] rbg);
}