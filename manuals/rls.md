# RLS

## ポリシーの作成

以下のポリシーを作成し、すべてのユーザーがSELECTクエリでusersテーブルの
データを取得できるように設定しました。

```sql
CREATE POLICY "Allow read access to all" ON public.trans_users
FOR SELECT
USING (true);
```

- 説明:
  - このポリシーは、USING (true) によって常にtrueを返し、全行に対して読み取り（SELECT）を許可する設定です。
  - この設定がなければ、RLSの制約によってアプリケーションはデータを取得できず、レスポンスが空になったり、エラーが発生したりします。


```sql
CREATE POLICY "Allow access to authenticated users" ON public.trans_users
FOR SELECT
USING (auth.uid() IS NOT NULL);
```

### SELECT（読み取り）のポリシー

```sql
CREATE POLICY "Allow access to authenticated users" ON public.trans_users
FOR SELECT
USING (auth.uid() IS NOT NULL);
```

### INSERT（挿入）のポリシー

```sql
CREATE POLICY "Allow access to authenticated users" ON public.trans_users
FOR INSERT
WITH CHECK (auth.uid() IS NOT NULL);
```

### UPDATE（更新）のポリシー

```sql
CREATE POLICY "Allow access to authenticated users" ON public.trans_users
FOR UPDATE
USING (auth.uid() IS NOT NULL);
```

### DELETE（削除）のポリシー

```sql
CREATE POLICY "Allow access to authenticated users" ON public.trans_users
FOR DELETE
USING (auth.uid() IS NOT NULL);
```

### ポリシーの有効化

```sql
ALTER TABLE public.trans_users ENABLE ROW LEVEL SECURITY;
```

### ポリシーの確認

```sql
SELECT * FROM supabase_security.policy;
```
