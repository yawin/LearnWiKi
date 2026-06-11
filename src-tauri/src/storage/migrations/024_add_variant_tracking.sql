-- Sprint 6A (E-6-3): Add variant tracking fields for wrong-answer variant generation
ALTER TABLE review_schedule ADD COLUMN variant_streak INTEGER DEFAULT 0;
ALTER TABLE review_schedule ADD COLUMN variant_mode INTEGER DEFAULT 0;

ALTER TABLE review_logs ADD COLUMN is_variant INTEGER DEFAULT 0;
ALTER TABLE review_logs ADD COLUMN variant_generation INTEGER DEFAULT 0;
