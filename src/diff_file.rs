//! This module handles printing a diff to the terminal of actual vs
//! expected output.
//!
//! This module has in large part been taken from `rustfmt`, including modifications
//!
//! Commit: 4b620cdbc5d5c1778c77c560f33f0f396ab878c1
//! Repo: https://github.com/rust-lang/rustfmt
//! License: MIT OR Apache-2.0
//!
//! MIT LICENSE
//! ===========
//!
//! Copyright (c) 2016-2021 The Rust Project Developers
//!
//! Permission is hereby granted, free of charge, to any
//! person obtaining a copy of this software and associated
//! documentation files (the "Software"), to deal in the
//! Software without restriction, including without
//! limitation the rights to use, copy, modify, merge,
//! publish, distribute, sublicense, and/or sell copies of
//! the Software, and to permit persons to whom the Software
//! is furnished to do so, subject to the following
//! conditions:
//!
//! The above copyright notice and this permission notice
//! shall be included in all copies or substantial portions
//! of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
//! ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
//! TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
//! PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
//! SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
//! CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
//! OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
//! IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
//! DEALINGS IN THE SOFTWARE.
//!
//! APACHE LICENSE
//! ==============
//!
//! Apache License
//! Version 2.0, January 2004
//! http://www.apache.org/licenses/
//!
//! TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION
//!
//! 1. Definitions.
//!
//! "License" shall mean the terms and conditions for use, reproduction,
//! and distribution as defined by Sections 1 through 9 of this document.
//!
//! "Licensor" shall mean the copyright owner or entity authorized by
//! the copyright owner that is granting the License.
//!
//! "Legal Entity" shall mean the union of the acting entity and all
//! other entities that control, are controlled by, or are under common
//! control with that entity. For the purposes of this definition,
//! "control" means (i) the power, direct or indirect, to cause the
//! direction or management of such entity, whether by contract or
//! otherwise, or (ii) ownership of fifty percent (50%) or more of the
//! outstanding shares, or (iii) beneficial ownership of such entity.
//!
//! "You" (or "Your") shall mean an individual or Legal Entity
//! exercising permissions granted by this License.
//!
//! "Source" form shall mean the preferred form for making modifications,
//! including but not limited to software source code, documentation
//! source, and configuration files.
//!
//! "Object" form shall mean any form resulting from mechanical
//! transformation or translation of a Source form, including but
//! not limited to compiled object code, generated documentation,
//! and conversions to other media types.
//!
//! "Work" shall mean the work of authorship, whether in Source or
//! Object form, made available under the License, as indicated by a
//! copyright notice that is included in or attached to the work
//! (an example is provided in the Appendix below).
//!
//! "Derivative Works" shall mean any work, whether in Source or Object
//! form, that is based on (or derived from) the Work and for which the
//! editorial revisions, annotations, elaborations, or other modifications
//! represent, as a whole, an original work of authorship. For the purposes
//! of this License, Derivative Works shall not include works that remain
//! separable from, or merely link (or bind by name) to the interfaces of,
//! the Work and Derivative Works thereof.
//!
//! "Contribution" shall mean any work of authorship, including
//! the original version of the Work and any modifications or additions
//! to that Work or Derivative Works thereof, that is intentionally
//! submitted to Licensor for inclusion in the Work by the copyright owner
//! or by an individual or Legal Entity authorized to submit on behalf of
//! the copyright owner. For the purposes of this definition, "submitted"
//! means any form of electronic, verbal, or written communication sent
//! to the Licensor or its representatives, including but not limited to
//! communication on electronic mailing lists, source code control systems,
//! and issue tracking systems that are managed by, or on behalf of, the
//! Licensor for the purpose of discussing and improving the Work, but
//! excluding communication that is conspicuously marked or otherwise
//! designated in writing by the copyright owner as "Not a Contribution."
//!
//! "Contributor" shall mean Licensor and any individual or Legal Entity
//! on behalf of whom a Contribution has been received by Licensor and
//! subsequently incorporated within the Work.
//!
//! 2. Grant of Copyright License. Subject to the terms and conditions of
//! this License, each Contributor hereby grants to You a perpetual,
//! worldwide, non-exclusive, no-charge, royalty-free, irrevocable
//! copyright license to reproduce, prepare Derivative Works of,
//! publicly display, publicly perform, sublicense, and distribute the
//! Work and such Derivative Works in Source or Object form.
//!
//! 3. Grant of Patent License. Subject to the terms and conditions of
//! this License, each Contributor hereby grants to You a perpetual,
//! worldwide, non-exclusive, no-charge, royalty-free, irrevocable
//! (except as stated in this section) patent license to make, have made,
//! use, offer to sell, sell, import, and otherwise transfer the Work,
//! where such license applies only to those patent claims licensable
//! by such Contributor that are necessarily infringed by their
//! Contribution(s) alone or by combination of their Contribution(s)
//! with the Work to which such Contribution(s) was submitted. If You
//! institute patent litigation against any entity (including a
//! cross-claim or counterclaim in a lawsuit) alleging that the Work
//! or a Contribution incorporated within the Work constitutes direct
//! or contributory patent infringement, then any patent licenses
//! granted to You under this License for that Work shall terminate
//! as of the date such litigation is filed.
//!
//! 4. Redistribution. You may reproduce and distribute copies of the
//! Work or Derivative Works thereof in any medium, with or without
//! modifications, and in Source or Object form, provided that You
//! meet the following conditions:
//!
//! (a) You must give any other recipients of the Work or
//! Derivative Works a copy of this License; and
//!
//! (b) You must cause any modified files to carry prominent notices
//! stating that You changed the files; and
//!
//! (c) You must retain, in the Source form of any Derivative Works
//! that You distribute, all copyright, patent, trademark, and
//! attribution notices from the Source form of the Work,
//! excluding those notices that do not pertain to any part of
//! the Derivative Works; and
//!
//! (d) If the Work includes a "NOTICE" text file as part of its
//! distribution, then any Derivative Works that You distribute must
//! include a readable copy of the attribution notices contained
//! within such NOTICE file, excluding those notices that do not
//! pertain to any part of the Derivative Works, in at least one
//! of the following places: within a NOTICE text file distributed
//! as part of the Derivative Works; within the Source form or
//! documentation, if provided along with the Derivative Works; or,
//! within a display generated by the Derivative Works, if and
//! wherever such third-party notices normally appear. The contents
//! of the NOTICE file are for informational purposes only and
//! do not modify the License. You may add Your own attribution
//! notices within Derivative Works that You distribute, alongside
//! or as an addendum to the NOTICE text from the Work, provided
//! that such additional attribution notices cannot be construed
//! as modifying the License.
//!
//! You may add Your own copyright statement to Your modifications and
//! may provide additional or different license terms and conditions
//! for use, reproduction, or distribution of Your modifications, or
//! for any such Derivative Works as a whole, provided Your use,
//! reproduction, and distribution of the Work otherwise complies with
//! the conditions stated in this License.
//!
//! 5. Submission of Contributions. Unless You explicitly state otherwise,
//! any Contribution intentionally submitted for inclusion in the Work
//! by You to the Licensor shall be under the terms and conditions of
//! this License, without any additional terms or conditions.
//! Notwithstanding the above, nothing herein shall supersede or modify
//! the terms of any separate license agreement you may have executed
//! with Licensor regarding such Contributions.
//!
//! 6. Trademarks. This License does not grant permission to use the trade
//! names, trademarks, service marks, or product names of the Licensor,
//! except as required for reasonable and customary use in describing the
//! origin of the Work and reproducing the content of the NOTICE file.
//!
//! 7. Disclaimer of Warranty. Unless required by applicable law or
//! agreed to in writing, Licensor provides the Work (and each
//! Contributor provides its Contributions) on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
//! implied, including, without limitation, any warranties or conditions
//! of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
//! PARTICULAR PURPOSE. You are solely responsible for determining the
//! appropriateness of using or redistributing the Work and assume any
//! risks associated with Your exercise of permissions under this License.
//!
//! 8. Limitation of Liability. In no event and under no legal theory,
//! whether in tort (including negligence), contract, or otherwise,
//! unless required by applicable law (such as deliberate and grossly
//! negligent acts) or agreed to in writing, shall any Contributor be
//! liable to You for damages, including any direct, indirect, special,
//! incidental, or consequential damages of any character arising as a
//! result of this License or out of the use or inability to use the
//! Work (including but not limited to damages for loss of goodwill,
//! work stoppage, computer failure or malfunction, or any and all
//! other commercial damages or losses), even if such Contributor
//! has been advised of the possibility of such damages.
//!
//! 9. Accepting Warranty or Additional Liability. While redistributing
//! the Work or Derivative Works thereof, You may choose to offer,
//! and charge a fee for, acceptance of support, warranty, indemnity,
//! or other liability obligations and/or rights consistent with this
//! License. However, in accepting such obligations, You may act only
//! on Your own behalf and on Your sole responsibility, not on behalf
//! of any other Contributor, and only if You agree to indemnify,
//! defend, and hold each Contributor harmless for any liability
//! incurred by, or claims asserted against, such Contributor by reason
//! of your accepting any such warranty or additional liability.
//!
//! END OF TERMS AND CONDITIONS
//!
//! APPENDIX: How to apply the Apache License to your work.
//!
//! To apply the Apache License to your work, attach the following
//! boilerplate notice, with the fields enclosed by brackets "[]"
//! replaced with your own identifying information. (Don't include
//! the brackets!)  The text should be enclosed in the appropriate
//! comment syntax for the file format. We also recommend that a
//! file or class name and description of purpose be included on the
//! same "printed page" as the copyright notice for easier
//! identification within third-party archives.
//!
//! Copyright 2016-2021 The Rust Project Developers
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//! http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.
//! use std::collections::VecDeque;
#![allow(clippy::doc_lazy_continuation, reason = "don't want to modify license")]

use std::collections::VecDeque;

// This struct handles writing output to stdout and abstracts away the logic
// of printing in color, if it's possible in the executing environment.
pub(crate) struct OutputWriter {
    terminal: Option<Box<dyn term::Terminal<Output = std::io::Stdout>>>,
}

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum Color {
    /// Always use color, whether it is a piped or terminal output
    Always,
    /// Never use color
    Never,
    /// Automatically use color, if supported by terminal
    Auto,
}

impl Color {
    /// Whether we should use a coloured terminal.
    pub fn use_colored_tty(self) -> bool {
        match self {
            Color::Always | Color::Auto => true,
            Color::Never => false,
        }
    }
}

impl OutputWriter {
    // Create a new OutputWriter instance based on the caller's preference
    // for colorized output and the capabilities of the terminal.
    pub(crate) fn new(color: Color) -> Self {
        if let Some(t) = term::stdout()
            && color.use_colored_tty()
            && t.supports_color()
        {
            return OutputWriter { terminal: Some(t) };
        }
        OutputWriter { terminal: None }
    }

    // Write output in the optionally specified color. The output is written
    // in the specified color if this OutputWriter instance contains a
    // Terminal in its `terminal` field.
    pub(crate) fn writeln(&mut self, msg: &str, color: Option<term::color::Color>) {
        match &mut self.terminal {
            Some(t) => {
                if let Some(color) = color {
                    t.fg(color).unwrap();
                }
                writeln!(t, "{msg}").unwrap();
                if color.is_some() {
                    t.reset().unwrap();
                }
            }
            None => println!("{msg}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum DiffLine {
    Context(String),
    Expected(String),
    Resulting(String),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Mismatch {
    /// The line number in the formatted version.
    pub(crate) line_number: u32,
    /// The line number in the original version.
    pub(crate) line_number_orig: u32,
    /// The set of lines (context and old/new) in the mismatch.
    pub(crate) lines: Vec<DiffLine>,
}

impl Mismatch {
    fn new(line_number: u32, line_number_orig: u32) -> Mismatch {
        Mismatch {
            line_number,
            line_number_orig,
            lines: Vec::new(),
        }
    }
}

// Produces a diff between the expected output and actual output of rustfmt.
pub(crate) fn make_diff(expected: &str, actual: &str, context_size: usize) -> Vec<Mismatch> {
    let mut line_number = 1;
    let mut line_number_orig = 1;
    let mut context_queue: VecDeque<&str> = VecDeque::with_capacity(context_size);
    let mut lines_since_mismatch = context_size + 1;
    let mut results = Vec::new();
    let mut mismatch = Mismatch::new(0, 0);

    for result in diff::lines(expected, actual) {
        match result {
            diff::Result::Left(str) => {
                if lines_since_mismatch >= context_size && lines_since_mismatch > 0 {
                    results.push(mismatch);
                    mismatch = Mismatch::new(
                        line_number - context_queue.len() as u32,
                        line_number_orig - context_queue.len() as u32,
                    );
                }

                while let Some(line) = context_queue.pop_front() {
                    mismatch.lines.push(DiffLine::Context(line.to_owned()));
                }

                mismatch.lines.push(DiffLine::Resulting(str.to_owned()));
                line_number_orig += 1;
                lines_since_mismatch = 0;
            }
            diff::Result::Right(str) => {
                if lines_since_mismatch >= context_size && lines_since_mismatch > 0 {
                    results.push(mismatch);
                    mismatch = Mismatch::new(
                        line_number - context_queue.len() as u32,
                        line_number_orig - context_queue.len() as u32,
                    );
                }

                while let Some(line) = context_queue.pop_front() {
                    mismatch.lines.push(DiffLine::Context(line.to_owned()));
                }

                mismatch.lines.push(DiffLine::Expected(str.to_owned()));
                line_number += 1;
                lines_since_mismatch = 0;
            }
            diff::Result::Both(str, _) => {
                if context_queue.len() >= context_size {
                    let _ = context_queue.pop_front();
                }

                if lines_since_mismatch < context_size {
                    mismatch.lines.push(DiffLine::Context(str.to_owned()));
                } else if context_size > 0 {
                    context_queue.push_back(str);
                }

                line_number += 1;
                line_number_orig += 1;
                lines_since_mismatch += 1;
            }
        }
    }

    results.push(mismatch);
    results.remove(0);

    results
}

pub(crate) fn print_diff<F>(diff: Vec<Mismatch>, get_section_title: F, color: Color)
where
    F: Fn(u32) -> String,
{
    let mut writer = OutputWriter::new(color);

    for mismatch in diff {
        let title = get_section_title(mismatch.line_number_orig);
        writer.writeln(&title, None);

        for line in mismatch.lines {
            match line {
                DiffLine::Context(ref str) => writer.writeln(&format!(" {str}"), None),
                DiffLine::Expected(ref str) => {
                    writer.writeln(&format!("+{str}"), Some(term::color::GREEN))
                }
                DiffLine::Resulting(ref str) => {
                    writer.writeln(&format!("-{str}"), Some(term::color::RED))
                }
            }
        }
    }
}
