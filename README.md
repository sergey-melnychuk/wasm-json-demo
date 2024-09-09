wasm-json-demo
==============

```
rm -rf web/.parcel-cache web/dist pkg target

wasm-pack build --target web
cd web
npx parcel build index.html
http-server ./dist
cd ..
```
