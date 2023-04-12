#!/bin/bash

set -e

VERSION=2.10.3
LIB=libxml2-${VERSION}
LIB_TAR=${LIB}.tar.xz

if [ -f ${LIB_TAR} ]; then
    echo ${LIB_TAR} downloaded
else
    echo downloading ${LIB_TAR} ...
    curl -L https://download.gnome.org/sources/libxml2/2.10/${LIB_TAR} -O
fi

if [ ! -f ${LIB_TAR} ]; then
    echo ${LIB_TAR} not found
fi

tar xf ${LIB_TAR}

pushd ${LIB}

if [ ! -f Makefile ]; then
    ./configure --disable-silent-rules --with-icu --without-python --without-lzma
    make -n | grep "compile gcc" > ../Makefile.output
fi

C_FILES=$(awk '/am__libxml2_la_SOURCES_DIST =/,/\.c$/' Makefile | tr '\n' ' ' | sed -E 's/am__libxml2_la_SOURCES_DIST = // ; s/\\//g ; s/[[:space:]]+/ /g ; s/ $//')

echo
echo ${C_FILES}
echo

find . $(printf "! -name %s " ${C_FILES}) ! -name '*.h' ! -name libxml2-api.xml -type f -exec rm {} +
rm trio*.*
find . ! -name . ! -name .. -type d -empty -delete
find . ! -name include ! -name doc -type d -depth 1 -exec rm -rf {} +

popd

pushd ..

echo -e "#[rustfmt::skip]\npub const C_FILES: &[&str] = &[" > VENDOR_C_FILES
find ./vendor -name "*.c" -exec echo "  concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{}\")," \; >> VENDOR_C_FILES
echo "];" >> VENDOR_C_FILES
mv VENDOR_C_FILES src/c_files.rs

popd

find ./${LIB}/include/libxml -name "*.h" | xargs basename | sed 's/\(.*\)/#include <libxml\/\1>/' > ../../xml2-sys/wrapper.h
