# Candidate introductions

Candidates for the [2025 Tauri Board elections](README.md).

## Jacob Bolda

I am Jacob Bolda, [@jbolda][github-jbolda]. Over the years, I led or was involved with many prototypes and first implementations of work we rely on today, such as `create-tauri-app`, the CI/CD and our documentation website. The "type" of time that I have been able to give has evolved with the project over the years, and in recent years, I have found my time best offered and valued in advising, managing, and also speaking about the project on my YouTube channel. Bringing folks in and keeping the project approachable will always be a requirement for the continued success of Tauri.

I assisted Robin in organizing and establishing the Tauri Programme about 4.5 years and joined as a Board Director. My efforts in organizing and advocating for improving management efforts began the year prior to that. I intend to continue these efforts to organize and establish the Tauri community in the most inclusive way I am able. A "theme" that we have been striving for is improving transparency of the organization and the onboarding into roles to further push the project forward.

Frontside Software is the consultancy to which I am employed. I work with as a software engineering consulting for large enterprise. My roles find great overlap in the work and needs of Tauri including engineering architecture, planning, and managing and executing large migrations. As a Director of the Board, I will continue to serve the Tauri community with these skills. We will hone our processes and workflows on the foundations we have established to help our domains achieve the best they are able.

[github-jbolda]: https://github.com/jbolda "Jacob Bolda - GitHub"

## Bill Avery

I am fortunate to have a job that I love doing writing code, so much so that I spend quite a bit of my free time working on OSS projects. Besides OSS projects, I also like to play games on my PC and viola in a community orchestra.

I work at Microsoft, originally in primarily in C++ codebases, but over the last few years I've been happily switching most new projects to Rust. Some of the first features I worked on were early hybrid web features which integrated WebView controls to host HTML UI with bindings to C++ in the host application. Since then, HTML, CSS, JavaScript, and browser APIs have become much more powerful, to the point where Progressive Web Applications and web hosts like Electron are competitive with native applications. If you throw in frameworks like React Native, it seems clear to me that the best future for most native client code bases is integrating with or into one of these JavaScript-based UI frameworks, rather than getting locked into a platform-specific native UI framework.

In 2021, I was looking into cross-platform WebView libraries, and simultaneously I discovered the [windows](https://github.com/microsoft/windows-rs) crate and the cross-platform Rust WebView bindings which later became [WRY](https://github.com/tauri-apps/wry). I figured Microsoft was missing an opportunity to define first class Rust bindings to the new WebView2 control on Windows, so I decided to try putting these projects together. The result was https://github.com/wravery/webview2-rs, along with a few PRs to WRY and TAO to perform an initial port from the hand-coded winapi crate to auto-generated first-party bindings using windows-rs.

Lately I've been working on auto-generated Rust bindings for [CEF](https://github.com/chromiumembedded/cef) (Chromium Embedded Framework) in [cef-rs](https://gitub.com/tauri-apps/cef-rs). Besides enriching the Rust ecosystem in general, my specific goal for this project is to integrate it with Tauri, potentially as an optional backend for [WRY](https://github.com/tauri-apps/wry). I want to close the performance and Chromium compatibility gap on macOS and Linux, where developers sometimes have no choice but to use Electron instead of Tauri. To be clear, continued work on this project doesn't depend on getting reelected to the board, I'm just mentioning it for familiarity with my work.

[github-wravery]: https://github.com/wravery "Bill Avery - GitHub"
