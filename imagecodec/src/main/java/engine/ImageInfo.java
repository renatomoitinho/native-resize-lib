package engine;

/**
 * @author renato.dias
 */
public class ImageInfo {

    private final int width;
    private final int height;
    private final long size;
    private final String mineType;
    private final String hash;

    public ImageInfo(int width, int height, long size, String mineType, String hash) {
        this.width = width;
        this.height = height;
        this.size = size;
        this.mineType = mineType;
        this.hash = hash;
    }

    public ImageInfo(int width, int height, long size) {
        this(width,height,size, null, null);
    }

    public int getWidth() {
        return width;
    }

    public int getHeight() {
        return height;
    }

    public long getSize() {
        return size;
    }

    public String getMineType() {
        return mineType;
    }

    public String getHash() {
        return hash;
    }
}
