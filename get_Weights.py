def get_Weights(stat_type):
    from sklearn.model_selection import train_test_split
    from sklearn.linear_model import LinearRegression
    from sklearn.preprocessing import StandardScaler
    import pandas as pd
    import numpy as np
    data_folder = '/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/data'
    team_stats = pd.read_csv('/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/team_stat.csv')
    if stat_type == 'attacking_stats':
        selected_columns = ['result', 'GF', 'xG']  # 选择的列
    elif stat_type == 'defense_stats':
        selected_columns = ['result', 'GA', 'xGA']  # 选择的列
    elif stat_type == 'keepers_stats':
        selected_columns = ['result', 'GA', 'xGA']  # 选择的列
    else:
        selected_columns = ['result', 'GF', 'GA', 'xG', 'xGA', 'Poss']  # 选择的列

    # 初始化一个空列表来存储防守数据的平均值
    defense_averages_list = []

    # 遍历每场比赛的game标识符，加载相应的防守数据
    for game in team_stats['game']:
        try:
            df = pd.read_csv(f'{data_folder}/{game}/{stat_type}.csv')
            df = df.drop(columns='player' ).drop(df.columns[0], axis=1)  # 排除player列
            defense_averages_list.append(df.mean())
        except FileNotFoundError:
            print(f"Defense stats for game {game} not found.")
            # 添加空数据以保持数据一致性
            defense_averages_list.append(pd.Series([None] * (len(df.columns) - 1), index=df.columns.drop('player')))

    # 从列表创建一个DataFrame
    defense_averages = pd.DataFrame(defense_averages_list)

    # 将防守数据的平均值添加到球队统计DataFrame中
    df = pd.concat([team_stats[selected_columns].reset_index(drop=True), defense_averages.reset_index(drop=True)], axis=1)
    
    x_columns = df.columns[len(selected_columns):]
    y_columns = df.columns[:len(selected_columns)]
    # 准备数据
    X = df[x_columns]
    Y = df[y_columns]
    X_train, X_test, Y_train, Y_test = train_test_split(X, Y, test_size=0.2, random_state=42)

    # 标准化特征
    scaler = StandardScaler()
    X_train_scaled = scaler.fit_transform(X_train)
    X_test_scaled = scaler.transform(X_test)

    # 拟合多目标回归模型
    model = LinearRegression()
    model.fit(X_train_scaled, Y_train)

    # 获取每个目标变量的系数
    coefficients = model.coef_

    feature_weights = {}
    # 计算每个特征对每个目标变量的影响权重
    for i, target in enumerate(y_columns):
        weights = np.abs(coefficients[i]) / np.sum(np.abs(coefficients[i]))
        for j, feature in enumerate(x_columns):
            if feature not in feature_weights:
                feature_weights[feature] = []
            feature_weights[feature].append(weights[j])

    # 计算每个特征的平均权重
    average_weights = {feature: np.mean(weights) for feature, weights in feature_weights.items()}
    return average_weights
