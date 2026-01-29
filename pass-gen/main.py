import _core
import pyperclip


def run():
    # text = _core.generate_password(32)
    text = _core.generate_passphrase(6)
    pyperclip.copy(text)


if __name__ == "__main__":
    run()
