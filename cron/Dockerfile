FROM ubuntu:16.04

# Install cron
RUN apt-get update && apt-get install -y cron postgresql postgresql-contrib

# Add files
ADD ./run.sh /run.sh
ADD ./entrypoint.sh /entrypoint.sh
 
RUN chmod +x /run.sh /entrypoint.sh

ENTRYPOINT /entrypoint.sh