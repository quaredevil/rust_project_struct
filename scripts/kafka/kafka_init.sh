#!/bin/bash
# Script to create Kafka topics for crypto-notifications service
# Aligned with Avro schemas in /schemas directory

set -e

KAFKA_BROKER="${KAFKA_BROKER:-kafka:9092}"
PARTITIONS="${KAFKA_PARTITIONS:-3}"
REPLICATION_FACTOR="${KAFKA_REPLICATION_FACTOR:-1}"

echo "Waiting for Kafka to be ready..."
cub kafka-ready -b "$KAFKA_BROKER" 1 60

echo "Creating Kafka topics..."

# Input topic (consumption)
kafka-topics --create \
  --bootstrap-server "$KAFKA_BROKER" \
  --topic crypto.notification \
  --partitions "$PARTITIONS" \
  --replication-factor "$REPLICATION_FACTOR" \
  --if-not-exists \
  --config retention.ms=604800000 \
  --config compression.type=snappy

echo "✓ Created topic: crypto.notification"

# Output topics (event publishing)
kafka-topics --create \
  --bootstrap-server "$KAFKA_BROKER" \
  --topic crypto.notification_delivered \
  --partitions "$PARTITIONS" \
  --replication-factor "$REPLICATION_FACTOR" \
  --if-not-exists \
  --config retention.ms=2592000000 \
  --config compression.type=snappy

echo "✓ Created topic: crypto.notification_delivered"

kafka-topics --create \
  --bootstrap-server "$KAFKA_BROKER" \
  --topic crypto.notification_failed \
  --partitions "$PARTITIONS" \
  --replication-factor "$REPLICATION_FACTOR" \
  --if-not-exists \
  --config retention.ms=2592000000 \
  --config compression.type=snappy

echo "✓ Created topic: crypto.notification_failed"

kafka-topics --create \
  --bootstrap-server "$KAFKA_BROKER" \
  --topic crypto.notification_throttled \
  --partitions "$PARTITIONS" \
  --replication-factor "$REPLICATION_FACTOR" \
  --if-not-exists \
  --config retention.ms=2592000000 \
  --config compression.type=snappy

echo "✓ Created topic: crypto.notification_throttled"

echo ""
echo "Listing all topics:"
kafka-topics --list --bootstrap-server "$KAFKA_BROKER"

echo ""
echo "✅ Kafka topics initialization completed successfully!"
