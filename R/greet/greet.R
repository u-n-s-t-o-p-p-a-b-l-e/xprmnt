#!/usr/bin/env Rscript

suppressPackageStartupMessages(library(optparse))

option_list <- list(
					make_option(c("-n", "--name"), type="character", default=NULL,
					help="Your name", metavar="character")
)
