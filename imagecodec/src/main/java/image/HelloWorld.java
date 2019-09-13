package image;

import java.io.Closeable;

public class HelloWorld implements ImageCodec, Closeable {

    private static native long createImageReference(byte[] input, HelloWorld callback);

    private static native void destroyReference(long reference_id);

    private static native byte[] resize(long reference_id, int width, int height, int quality, String format);

    private static native byte[] scale(long reference_id, int width, int height, int quality, String format);

    static {
        System.loadLibrary("mylib");
    }

    public HelloWorld() {
    }

    public HelloWorld(byte[] bytes) {
        setReferenceObject(createImageReference(bytes, this));
        setSize(bytes.length);
    }

    private int width;
    private int height;
    private int size;
    private long referenceObject;

    public int getWidth() {
        return width;
    }

    public void setWidth(int width) {
        this.width = width;
    }

    public int getHeight() {
        return height;
    }

    public void setHeight(int height) {
        this.height = height;
    }

    public int getSize() {
        return size;
    }

    public void setSize(int size) {
        this.size = size;
    }

    private void setReferenceObject(long referenceObject) {
        this.referenceObject = referenceObject;
    }

    @Override
    public byte[] resize(int width, int height, int quality, IMG_FORMAT img_format) {
        byte[] bytes = resize(this.referenceObject, width, height, quality, img_format.name());
        setSize(bytes.length);
        return bytes;
    }

    @Override
    public byte[] scale(int width, int height, int quality, IMG_FORMAT img_format) {
        byte[] bytes = scale(this.referenceObject, width, height, quality, img_format.name());
        setSize(bytes.length);
        return bytes;
    }

    @Override
    public void close() {
        destroyReference(this.referenceObject);
    }

    @Override
    public String toString() {
        return "{" +
                "  width=" + width +
                ", height=" + height +
                ", size=" + size +
                ", referenceObject=" + referenceObject +
                '}';
    }
}
