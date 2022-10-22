############
# Install  #
############

install:
  @cargo build --release
  sudo cp ./target/release/pam-template /bin/pam-template
