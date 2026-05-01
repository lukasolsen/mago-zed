<?php

declare(strict_types=1);

$page = $_GET["page"];
include "pages/" . $page . ".php"; // LFI
require $_GET["template"]; // RFI
