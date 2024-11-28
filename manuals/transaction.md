
```sql
-- トランザクション開始関数
CREATE OR REPLACE FUNCTION public.begin_transaction()
RETURNS void AS $$
BEGIN
    PERFORM set_config('tx.started', 'true', true);
END;
$$ LANGUAGE plpgsql;

-- トランザクションコミット関数
CREATE OR REPLACE FUNCTION public.commit_transaction()
RETURNS void AS $$
BEGIN
    -- コミットは自動的に行われるため、特に何もしない
    RETURN;
END;
$$ LANGUAGE plpgsql;

-- トランザクションロールバック関数
CREATE OR REPLACE FUNCTION public.rollback_transaction()
RETURNS void AS $$
BEGIN
    RAISE EXCEPTION 'ROLLBACK';
END;
$$ LANGUAGE plpgsql;
```