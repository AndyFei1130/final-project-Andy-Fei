import pandas as pd
import soccerdata as sd
import numpy as np
import os
pd.set_option('display.max_columns', None)

fbref = sd.FBref(leagues="ENG-Premier League", seasons=2022)
df = fbref.read_team_match_stats(stat_type="schedule", team='Arsenal')
df = df.drop(columns=['date','time','round', 'day', 'Attendance', 'Captain', 'Formation', 'Referee', 'Notes'])
df['venue'] = df['venue'].apply(lambda x: 1 if x == 'Home' else 0)
df['result'] = np.where(df['result'] == 'W', 3,
                       np.where(df['result'] == 'L', 0, 1))
df['match_report'] = df['match_report'].apply(lambda x: x.split('/')[3])

def get_player_data(id):
    lineups = fbref.read_lineup(match_id=id)
    starters = lineups[(lineups['team'] == 'Arsenal') & (lineups['is_starter'] == True)]
    starters_position = {'keeper': [], 'def': [], 'mid': [], 'att': []}
    for index, player in starters.iterrows():
        position = player['position'].split(',')[-1].strip()  # 取最后一个位置
        position = position[-1]  # 取位置的最后一个字母
        if position == 'K':
            starters_position['keeper'].append(player['player'])
        elif position == 'B':
            starters_position['def'].append(player['player'])
        elif position == 'M':
            starters_position['mid'].append(player['player'])
        elif position == 'W':
            starters_position['att'].append(player['player'])

    keepers_stats = fbref.read_player_match_stats(stat_type="keepers", match_id=id)
    defense_stats = fbref.read_player_match_stats(stat_type="defense", match_id=id)
    passing_stats = fbref.read_player_match_stats(stat_type="passing", match_id=id)
    attacking_stats = fbref.read_player_match_stats(stat_type="summary", match_id=id)

    folder_name = str(keepers_stats.index.get_level_values(2)[0])
    # 删除不符合条件的行
    keepers_stats = keepers_stats[keepers_stats.index.get_level_values(-1).isin(starters_position['keeper'])].droplevel(level=0,axis=1).drop(columns='')
    defense_stats = defense_stats[defense_stats.index.get_level_values(-1).isin(starters_position['def'])].droplevel(level=0,axis=1).drop(columns='')
    passing_stats = passing_stats[passing_stats.index.get_level_values(-1).isin(starters_position['mid'])].droplevel(level=0,axis=1).drop(columns='')
    attacking_stats = attacking_stats[attacking_stats.index.get_level_values(-1).isin(starters_position['att'])].droplevel(level=0,axis=1).drop(columns='')

    defense_stats = defense_stats.loc[:, ~defense_stats.columns.duplicated()].drop(columns=defense_stats.columns[defense_stats.columns.str.contains('%|#|\(|\)')]).reset_index().drop(['league', 'season', 'game', 'team'], axis=1)
    attacking_stats = attacking_stats.loc[:, ~attacking_stats.columns.duplicated()].drop(columns=attacking_stats.columns[attacking_stats.columns.str.contains('%|#|\(|\)')]).reset_index().drop(['league', 'season', 'game', 'team'], axis=1)
    passing_stats = passing_stats.loc[:, ~passing_stats.columns.duplicated()].drop(columns=passing_stats.columns[passing_stats.columns.str.contains('%|#|\(|\)')]).reset_index().drop(['league', 'season', 'game', 'team'], axis=1)
    keepers_stats = keepers_stats.loc[:, ~keepers_stats.columns.duplicated()].drop(columns=keepers_stats.columns[keepers_stats.columns.str.contains('%|#|\(|\)')]).reset_index().drop(['league', 'season', 'game', 'team'], axis=1)
    
    defense_stats.columns = defense_stats.columns.str.lower()
    attacking_stats.columns = attacking_stats.columns.str.lower()
    passing_stats.columns = passing_stats.columns.str.lower()
    keepers_stats.columns = keepers_stats.columns.str.lower()

    defense_stats.rename(columns=lambda x: x.replace(" ", ""), inplace=True)
    attacking_stats.rename(columns=lambda x: x.replace(" ", ""), inplace=True)
    passing_stats.rename(columns=lambda x: x.replace(" ", ""), inplace=True)
    keepers_stats.rename(columns=lambda x: x.replace(" ", ""), inplace=True)

    # 将文件夹名称转换为字符串
    print(folder_name)
    # 指定文件夹路径并保存 CSV 文件
    keepers_stats.to_csv(f'/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/data/{folder_name}/keepers_stats.csv')
    defense_stats.to_csv(f'/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/data/{folder_name}/defense_stats.csv')
    passing_stats.to_csv(f'/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/data/{folder_name}/passing_stats.csv')
    attacking_stats.to_csv(f'/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/data/{folder_name}/attacking_stats.csv')

    
# for i in range(df.shape[0]):
#     folder_name = str(df.index.get_level_values(-1)[i])
#     try:
#         os.makedirs(folder_name)
#         print(f"文件夹 '{folder_name}' 创建成功")
#     except FileExistsError:
#         print(f"文件夹 '{folder_name}' 已存在")


for i in range(df.shape[0]):
    get_player_data(str(df['match_report'][i]))

# df.to_csv(f'/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/team_stat.csv')




