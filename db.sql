CREATE TABLE IF NOT EXISTS events (
  id UUID DEFAULT uuid_generate_v4() primary key,
  data JSONB NOT NULL,
  context JSONB DEFAULT '{}',
  time TIMESTAMP DEFAULT now()
);
