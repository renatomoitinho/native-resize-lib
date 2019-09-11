import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

class HelloWorld {

    private static native String hello(String input);
    private static native byte[] helloByte(byte[] input);
    private static native void factAndCallMeBack(int n, HelloWorld callback);
    private static native long counterNew(HelloWorld callback);
    private static native void counterIncrement(long counter_ptr);
    private static native void asyncComputation(HelloWorld callback);
    private static native void counterDestroy(long counter_ptr);

    // images

    private static native long createImageReference(byte[] input, HelloWorld callback);

    static {
        System.loadLibrary("mylib");
    }

    public HelloWorld() {
    }

    public HelloWorld(byte[] bytes) {
        setReferenceObject(createImageReference(bytes, this));
    }

    public static void main(String[] args) throws IOException {

        byte[] bytes = Files.readAllBytes(Paths.get("/Users/renatomoitinho/Documents/repositories/rust-lang/imgs/10mb.jpg"));

        HelloWorld imageRef = new HelloWorld(bytes);

        System.out.println("image => " + imageRef.toString() );

        String[] names = {
                "Aurore Muirgel",
                "Jalil Arun",
                "Guilherme Hróaldr",
                "Zemfira Roshan",
                "Abhijit Meine",
        };

        for(int i=0; i < names.length ;i++) {
            String output = HelloWorld.hello(names[i]);
            System.out.println(output);
        }

        long counter_ptr = counterNew(new HelloWorld());

        for (int i = 0; i < 5; i++) {
            counterIncrement(counter_ptr);
        }

        counterDestroy(counter_ptr);

        byte[] outputByte = HelloWorld.helloByte("byte".getBytes());
        System.out.println(outputByte);

        HelloWorld.factAndCallMeBack(6, new HelloWorld());

        System.out.println("Invoking asyncComputation (thread id = " + Thread.currentThread().getId() + ")");
        asyncComputation(new HelloWorld());
    }

    public void factCallback(int res) {
        System.out.println("factCallback: res = " + res);
    }

    public void counterCallback(int count) {
        System.out.println("counterCallback: count = " + count);
    }

    public void asyncCallback(int progress) {
        System.out.println("asyncCallback: thread id = " + Thread.currentThread().getId() + ", progress = " + progress + "%");
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

        public long getReferenceObject() {
            return referenceObject;
        }

        public void setReferenceObject(long referenceObject) {
            this.referenceObject = referenceObject;
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