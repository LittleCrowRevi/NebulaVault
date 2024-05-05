
using System;
using UnityEngine;

public class ResourceLoader : MonoBehaviour
{
    [Header( "Resources" )]
    public GameObject BattleUiPrefab;

    public GameObject OverworldUiPrefab;

    public GameObject StatScreenPrefab;
    
    [Header( "Listen to Event" )]
    [SerializeField] public VoidEventChannelSO m_LoadBattleUi;

    private void Start()
    {
        DontDestroyOnLoad( gameObject );
        
        m_LoadBattleUi.OnEventRaised += OnLoadBattleUi;
    }
    
    private void OnLoadBattleUi()
    {
        Instantiate( BattleUiPrefab );
    }
}