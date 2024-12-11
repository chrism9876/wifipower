Tool to enable wifi if no lan and to disable wifi if connected to lan.


Designed to run from lauchDeamon



### to crosscompile\
    ```
    rustup target add aarch64-apple-darwin  
    ```

    ```
    cargo build --release --target aarch64-apple-darwin
    ```

Universal binary created wih lipo \
    `lipo -create -output universal intelbinary arm binary`


# Pkg intaller
generated with packages.
Includes LauchDeamon to run every 5 minutes this is customisable