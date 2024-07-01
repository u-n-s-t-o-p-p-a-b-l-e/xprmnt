#!/usr/bin/env Rscript

suppressPackageStartupMessages(library(optparse))

option_list <- list(
					make_option(c("-n", "--name"), type="character", default=NULL,
					help="Your name", metavar="character")
)

opt_parser <- OptionParser(option_list=option_list)
opt <- parse_args(opt_parser)

greet_user <- function(name) {
	if (is.null(name)) {
		cat("Hello! Please provide your name using -n or --name option.\n")
	} else {
		cat("Hi, ", name, "!\n", sep = "")
	}
}

greet_user(opt$name)
