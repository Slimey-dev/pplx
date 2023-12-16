# PPLX CHAT APP

This is a local chat application built with Tauri and the PPLX API.

## Description

This application allows users to chat locally on their machine. It uses the PPLX API to facilitate the chat functionality and is built with Tauri, a framework for building lightweight, secure, and cross-platform desktop applications with web technologies.

## Installation

To install the application, follow these steps:

1. Clone the repository: `git clone https://github.com/Slimey-dev/pplx`
2. Install the dependencies: `pnpm i`
3. Build the application: `pnpm tauri build`

## Usage

After building the application, you can run it locally on your machine. Open settings and paste your api key into the first field. Enter a message into the chat input and press enter to send it. You can also use the send button to send a message. To clear the chat, press the bin button. If you generate a code with the app you can copy it by pressing the clipboard icon in the top right corner of the codeblock.

## Settings

The settings are stored in a file called `config.json` in the same directory as the executable. You can change your api key, if you want to exit on close and model in the settings.

## Contributing

Contributions are welcome. Please open an issue or submit a pull request.

## License

[GPL 3.0](https://choosealicense.com/licenses/gpl-3.0/)
