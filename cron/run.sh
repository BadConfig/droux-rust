psql postgres://docker:docker@postgres1:5432/diesel_db -c "UPDATE products SET status = 'deleted' WHERE (EXTRACT(EPOCH FROM current_timestamp-create_datetime)/(60*60*60) > 60) AND status = 'published'; DELETE FROM views;"