#!/usr/bin/perl
use strict;
use warnings;

if (@ARGV != 1) {
	print "Usage: $0 <name>\n";
	exit 1;
}

my $name = $ARGV[0];
print "Hello, $name!\n";
