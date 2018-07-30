#!/bin/bash

psql -h localhost -p 5431 -U postgres -d organisations-rs < db.sql


# vim: set ts=2 sw=2 et:


