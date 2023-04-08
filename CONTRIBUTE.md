# XML2 bindings in Rust development

## Development

To change version and update source tree and `wrapper.h` use `xml2-src/vendor/prepare.sh`.

To refresh bindings from current dynamically generated version:

```sh
find target -name bindings.rs -exec cp {} xml2-sys/ \;
```
