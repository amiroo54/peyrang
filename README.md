# svg-pallet-changer

A desktop app built using Rust with Tauri and Vue.js that allows you to change the color palette of SVG images.

## Features

- Load an SVG file from your local machine
- Select and change the colors within the SVG image
- Currently three modes available, Replace, Permutate and hue set

## Technologies Used

- Rust with Tauri for building the native desktop application
- Vue.js for the frontend user interface

## Getting Started

To run in dev mode, follow these steps:

1. Clone the repository.
2. Install Rust and Tauri if you haven't already.
3. Install Node.js and npm for managing the Vue.js frontend.
4. Install the necessary dependencies by running npm install in the project directory.
5. Run `cargo tauri dev`

## Usage

After running the app, you can select an SVG file using the load SVG button. Then the SVG file and the colors of it will be loaded. You can now select select the colors and choose one of the operations. The result will be saved in a folder called output in the folder where the original SVG file were.

### Replace

This will replace each color of the image with the selected coresponding color. 

### Permutate

This will generate n! images where n is the number of colors selected. Use with caution.

### Hue set

This will set the hue of all colors in the SVG file to the first color in the list using OKLab color space.

## Contributing

If you want to contribute to this project, feel free to fork the repository and submit a pull request with your proposed changes. We welcome any contributions that improve the functionality or usability of the application.

## License

This project is licensed under the MIT License. Feel free to use, modify, and distribute the code as long as you include the original license file.

## Support

If you encounter any issues or have questions about using the application, please feel free to open an issue in the repository or reach out to the maintainers for assistance.
