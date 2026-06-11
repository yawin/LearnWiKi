-- Migration 031: Drop old learning system tables
-- Replaced by Goal-based learning (migrations 027-030)
-- Old data retained in backup before running this migration

DROP TABLE IF EXISTS task_daily_logs;
DROP TABLE IF EXISTS practice_tasks;
DROP TABLE IF EXISTS modules;
DROP TABLE IF EXISTS learning_paths;
