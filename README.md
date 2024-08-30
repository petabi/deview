# Deview

Deview (Dioxus + REview) is Petabi's internal tool designed to streamline the
development and testing process for REview, our central management server that
collects and analyzes events. Deview creates a modern, efficient web interface
using Dioxus with server-side rendering (SSR) to interact with REview.

## Project Goals

1. **Improve Internal Testing**: Provide our team with a dedicated tool to test
   REview's functionality without needing to set up a full REview instance.
2. **Showcase Modern Web Technologies**: Demonstrate the advantages of using
   Dioxus and SSR over the current client-side rendering approach in the
   official REview UI.
3. **Enhance Performance**: Utilize Dioxus and SSR to significantly reduce
   initial load times and improve overall application responsiveness.
4. **Support Cloud Deployments**: Prepare REview for cloud-based deployments by
   addressing potential performance bottlenecks and improving scalability.

## Building and Running

This section will guide you through setting up Deview for development. Whether
you're experienced with web application development in Rust or new to the field,
these instructions will help you get started.

### Prerequisites

Before you begin, ensure you have the following tools installed:

1. **Dioxus CLI**: The Dioxus CLI tool is necessary for building and running
   Deview. Install it using Cargo:

   ```sh
   cargo install dioxus-cli
   ```

2. **npm**: If you don't already have npm installed, you can install it via
   Homebrew:

   ```sh
   brew install npm
   ```

3. **Tailwind CLI**: Tailwind CSS is used for styling in Deview. Install the
   Tailwind CLI with the following command:

   ```sh
   brew install tailwindcss
   ```

### Building and Running Deview

With the prerequisites installed, follow these steps to build and run Deview:

1. **Start the Tailwind CLI build process**: This command will watch for changes
   in your CSS and rebuild the Tailwind output automatically.

   ```sh
   npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
   ```

2. **Launch Deview**: In a separate terminal window, use the Dioxus CLI to start
   the Deview development server.

   ```sh
   dx serve --platform fullstack
   ```

3. **Access Deview in your browser**: Open your web browser and navigate to
   [http://127.0.0.1:8080](http://127.0.0.1:8080) to see Deview in action.

### Troubleshooting Tips

- **Dioxus CLI Not Found**: If you encounter an error about the Dioxus CLI not
  being found, ensure it was installed correctly using Cargo.
- **Port Already in Use**: If port 8080 is in use, you can specify a different
  port by adding the `--port` option when launching Deview:

  ```sh
  dx serve --platform fullstack --port 8081
  ```

## License

Copyright 2024 Petabi, Inc.

Licensed under the [Apache License, Version 2.0](apache-license) (the
"License"); you may not use this crate except in compliance with the License.

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the
[License][apache-license] for the specific language governing permissions and
limitations under the License.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Deview by you, as defined in the [Apache-2.0
license][apache-license], shall be licensed under
the same license, without any additional terms or conditions.

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0
