import _core
import pyperclip


def run():
    text = _core.generate_password()
    pyperclip.copy(text)


if __name__ == "__main__":
    run()
