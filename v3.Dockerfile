FROM rust:1.37

RUN apt-get update
RUN apt-get install -y software-properties-common

#opencv deps
RUN apt-get -y install build-essential cmake git libgtk2.0-dev pkg-config libavcodec-dev libavformat-dev libswscale-dev libtbb2 libtbb-dev libjpeg-dev libpng-dev libtiff-dev libdc1394-22-dev locales python-dev python-numpy libpng-dev clang libclang1

# DOWNLOADS

# DOWNLOAD libjpeg-turbo
RUN wget https://ufpr.dl.sourceforge.net/project/libjpeg-turbo/2.0.0/libjpeg-turbo-2.0.0.tar.gz -O libjpeg-turbo.tar.gz
RUN mkdir libjpeg-turbo
RUN tar -xzf libjpeg-turbo.tar.gz -C libjpeg-turbo --strip-components=1

# DOWNLOAD Opencv contrib
RUN wget https://github.com/opencv/opencv_contrib/archive/3.4.7.tar.gz -O opencv_contrib.tar.gz
RUN mkdir opencv_controlib
RUN tar -xzf opencv_contrib.tar.gz -C opencv_controlib --strip-components=1

#DOWNLOAD Opencv
RUN wget https://github.com/opencv/opencv/archive/3.4.7.tar.gz -O opencv.tar.gz
RUN mkdir opencv
RUN tar -xzf opencv.tar.gz -C opencv --strip-components=1

# INSTALL libjpeg-turbo
ENV CFLAGS "-fPIC -O3"
ENV CXXFLAGS "-fPIC -O3"

RUN apt-get -y install autoconf automake libtool nasm
RUN mkdir libjpeg-turbo/build
RUN cd libjpeg-turbo/build && cmake -DCMAKE_INSTALL_PREFIX=/usr/local -DCMAKE_BUILD_TYPE=RELEASE -DCMAKE_INSTALL_DEFAULT_LIBDIR=lib ..
RUN cd libjpeg-turbo/build && make
RUN cd libjpeg-turbo/build && make install

# INSTALL Opencv

RUN mkdir opencv/build
RUN cd opencv/build
RUN cd opencv/build && cmake -DENABLE_PRECOMPILED_HEADERS=OFF -DJPEG_INCLUDE_DIR=/usr/local/include/ -DJPEG_LIBRARY=/usr/local/lib/libturbojpeg.a -DOPENCV_EXTRA_MODULES_PATH=/opencv_controlib/modules -DCMAKE_BUILD_TYPE=Release -DBUILD_PERF_TESTS=OFF -DBUILD_TESTS=OFF -DWITH_JPEG=ON -DINSTALL_TESTS=OFF -DOPENCV_GENERATE_PKGCONFIG=ON -DBUILD_DOCS=OFF -DBUILD_EXAMPLES=OFF -DBUILD_opencv_apps=ALL -DWITH_IPP=OFF -DPYTHON_EXECUTABLE=OFF -DINSTALL_PYTHON_EXAMPLES=OFF -DWITH_LAPACK=ON -DWITH_EIGEN=ON -DBUILD_SHARED_LIBS=ON -DWITH_TBB=ON -DOPENCV_ENABLE_NONFREE=ON -DCMAKE_INSTALL_PREFIX=/usr/local ..
RUN cd opencv/build && make -j7
RUN cd opencv/build && make install

# Set the locale
RUN locale-gen en_US.UTF-8
RUN localedef -i en_US -f UTF-8 en_US.UTF-8
RUN sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && locale-gen
ENV LANG en_US.UTF-8 
ENV LANGUAGE en_US:en  
ENV LC_ALL en_US.UTF-8
ENV PKG_CONFIG_PATH /usr/local/lib/pkgconfig
ENV LD_LIBRARY_PATH /usr/local/lib

#executable
RUN apt-get update
RUN apt-get install -y software-properties-common
RUN apt-get -y install netcat
CMD nc -lc  5060
