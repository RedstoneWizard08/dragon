package sass

func (compiler *SassCompiler) IOOptions() string {
	empty := SassCompiler{}
	opts := ""

	if compiler.LoadPath != empty.LoadPath {
		opts += "--load-path=" + compiler.LoadPath + " "
	}

	if compiler.OutputStyle != empty.OutputStyle {
		opts += "--style=" + compiler.OutputStyle.String() + " "
	}

	if compiler.EmitCharset != empty.EmitCharset {
		if compiler.EmitCharset {
			opts += "--charset "
		} else {
			opts += "--no-charset "
		}
	}

	if compiler.EmitErrorCSS != empty.EmitErrorCSS {
		if compiler.EmitErrorCSS {
			opts += "--error-css "
		} else {
			opts += "--no-error-css "
		}
	}

	if compiler.UpdateMode != empty.UpdateMode {
		if compiler.UpdateMode {
			opts += "--update "
		}
	}

	if compiler.StopOnError != empty.StopOnError {
		if compiler.StopOnError {
			opts += "--stop-on-error "
		} else {
			opts += "--no-stop-on-error "
		}
	}

	return opts
}

func (compiler *SassCompiler) SourceMapOptions() string {
	empty := SassCompiler{}.SourceMap
	opts := compiler.SourceMap
	cmd := ""

	if opts.Enable != empty.Enable {
		if opts.Enable {
			cmd += "--source-map "
		} else {
			cmd += "--no-source-map "
		}
	}

	if opts.UrlLinkingType != empty.UrlLinkingType {
		cmd += "--source-map-urls=" + opts.UrlLinkingType.String() + " "
	}

	if opts.EmbedSourceMapsInCSS != empty.EmbedSourceMapsInCSS {
		if opts.EmbedSourceMapsInCSS {
			cmd += "--embed-source-map "
		} else {
			cmd += "--no-embed-source-map "
		}
	}

	if opts.EmbedSourcesInMaps != empty.EmbedSourcesInMaps {
		if opts.EmbedSourcesInMaps {
			cmd += "--embed-sources "
		} else {
			cmd += "--no-embed-sources "
		}
	}

	return cmd
}

func (compiler *SassCompiler) WatchOptions() string {
	empty := SassCompiler{}.Watch
	opts := compiler.Watch
	cmd := ""

	if opts.Enable != empty.Enable {
		if opts.Enable {
			cmd += "--watch "
		} else {
			cmd += "--watch "
		}
	}

	if opts.UsePolling != empty.UsePolling {
		if opts.UsePolling {
			cmd += "--poll "
		} else {
			cmd += "--no-poll "
		}
	}

	return cmd
}
