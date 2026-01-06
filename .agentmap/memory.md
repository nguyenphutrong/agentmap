# Memory

This file contains extracted knowledge markers from the codebase.

## Summary

| Category | Count | High | Medium | Low |
| -------- | ----- | ---- | ------ | --- |
| ⚠️ Warnings | 3 | 3 | 0 | 0 |
| 📋 Business Rules | 2 | 2 | 0 | 0 |
| 🔧 Technical Debt | 3 | 0 | 3 | 0 |
| 📝 Notes | 1 | 0 | 0 | 1 |

---

## ⚠️ Warnings

### 🔴 `WARNING` (order.php:2)

> This file handles payment processing - be careful with changes

### 🔴 `SAFETY` (order.php:38)

> Must check inventory before payment

### 🔴 `WARNING` (order.php:61)

> Deleting orders affects financial reports

---

## 📋 Business Rules

### 🔴 `RULE` (order.php:3)

> All order modifications must go through validateOrder() first

### 🔴 `RULE` (order.php:90)

> Refunds require manager approval for amounts > $500

---

## 🔧 Technical Debt

### 🟡 `TODO` (order.php:12)

> Split this file into smaller controllers

### 🟡 `FIXME` (order.php:23)

> This method is too long, needs refactoring

### 🟡 `TODO` (order.php:54)

> Add permission check here

---

## 📝 Notes

### 🟢 `NOTE` (order.php:82)

> This is used for reporting only

---

